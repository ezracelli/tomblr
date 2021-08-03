use std::fmt::{self, Display};
use std::str::FromStr;

use diesel::backend::Backend;
use diesel::prelude::*;
use diesel::sql_types::Text;
use diesel::types::{FromSql, ToSql};
use graphql::Context;

use crate::schema::blogs;
use crate::schema::email_accounts;
use crate::schema::oauth_accounts;
use crate::schema::posts;
use crate::schema::users;

pub trait Node
where
    Self: graphql::OutputType,
{
    fn cursor(&self) -> Cursor;
}

#[derive(
    Debug, diesel::Associations, diesel::Identifiable, diesel::Queryable, graphql::SimpleObject,
)]
#[belongs_to(User)]
#[graphql(complex)]
pub struct Blog {
    #[graphql(skip)]
    pub _rowid: i32,
    pub created_at: DateTime,
    #[graphql(skip)]
    pub deleted_at: Option<DateTime>,
    pub id: uuid::Uuid,
    pub slug: String,
    pub title: String,
    pub updated_at: DateTime,
    #[graphql(skip)]
    pub user_id: uuid::Uuid,
}

#[graphql::ComplexObject]
impl Blog {
    pub async fn posts(
        &self,
        ctx: &Context<'_>,
        first: i64,
        after: Option<Cursor>,
    ) -> graphql::Result<Connection<Post>> {
        let after = after.unwrap_or_default();

        let pool = ctx.data_unchecked::<crate::db::Pool>();
        let nodes: Vec<Post> = Post::belonging_to(self)
            .filter(crate::schema::posts::_rowid.gt(after._rowid))
            .limit(first)
            .get_results(&pool.get()?)?;

        Ok(Connection {
            edges: nodes.into_iter().map(Edge::from).collect(),
            page_info: PageInfo {
                has_next_page: crate::schema::posts::table
                    .count()
                    .get_result::<i64>(&pool.get()?)?
                    > (after._rowid as i64 + 1),
                has_previous_page: after._rowid > 0,
            },
        })
    }

    pub async fn user(&self, ctx: &Context<'_>) -> graphql::Result<User> {
        let pool = ctx.data_unchecked::<crate::db::Pool>();

        Ok(users::table.find(self.user_id).get_result(&pool.get()?)?)
    }
}

impl Node for Blog {
    fn cursor(&self) -> Cursor {
        Cursor {
            _rowid: self._rowid,
            ty: String::from("Blog"),
        }
    }
}

#[derive(Debug, diesel::Insertable, graphql::InputObject)]
#[table_name = "blogs"]
pub struct BlogCreateInput {
    pub slug: String,
    pub title: String,
    pub user_id: uuid::Uuid,
}

#[derive(Debug, graphql::SimpleObject)]
pub struct BlogCreateOutput {
    pub blog: Blog,
}

#[derive(Debug, graphql::SimpleObject)]
#[graphql(concrete(name = "BlogConnection", params(Blog)))]
#[graphql(concrete(name = "PostConnection", params(Post)))]
#[graphql(concrete(name = "UserConnection", params(User)))]
pub struct Connection<T: Node>
where
    Edge<T>: graphql::OutputType,
{
    pub edges: Vec<Edge<T>>,
    pub page_info: PageInfo,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Cursor {
    pub _rowid: i32,
    pub ty: String,
}

#[graphql::Scalar]
impl graphql::ScalarType for Cursor {
    fn parse(value: graphql::Value) -> graphql::InputValueResult<Self> {
        if let graphql::Value::String(value) = value {
            let value = base64::decode(value)?;
            Ok(serde_json::from_slice(&*value)?)
        } else {
            Err(graphql::InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> graphql::Value {
        graphql::Value::String(base64::encode(serde_json::to_vec(&self).unwrap()))
    }
}

pub type DateTime = chrono::DateTime<chrono::Utc>;

#[derive(Debug, graphql::SimpleObject)]
#[graphql(concrete(name = "BlogEdge", params(Blog)))]
#[graphql(concrete(name = "PostEdge", params(Post)))]
#[graphql(concrete(name = "UserEdge", params(User)))]
pub struct Edge<T: Node> {
    pub cursor: Cursor,
    pub node: T,
}

impl<T: Node> From<T> for Edge<T> {
    fn from(node: T) -> Self {
        Self {
            cursor: node.cursor(),
            node,
        }
    }
}

#[derive(
    Debug, diesel::Associations, diesel::Identifiable, diesel::Queryable, graphql::SimpleObject,
)]
#[belongs_to(User)]
pub struct EmailAccount {
    #[graphql(skip)]
    pub _rowid: i32,
    pub created_at: DateTime,
    #[graphql(skip)]
    pub deleted_at: Option<DateTime>,
    pub id: uuid::Uuid,
    pub provider_account_id: String,
    pub updated_at: DateTime,
    #[graphql(skip)]
    pub user_id: uuid::Uuid,
}

#[derive(Debug, diesel::Insertable, graphql::InputObject)]
#[table_name = "email_accounts"]
pub struct EmailAccountCreateInput {
    pub provider_account_id: String,
    pub user_id: uuid::Uuid,
}

#[derive(Debug, graphql::SimpleObject)]
pub struct EmailAccountCreateOutput {
    pub email_account: EmailAccount,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, diesel::AsExpression, diesel::FromSqlRow, graphql::Enum,
)]
#[graphql(name = "OAuthAccountProvider")]
#[sql_type = "Text"]
pub enum OAuthAccountProvider {
    APPLE,
    GITHUB,
    GOOGLE,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseOAuthAccountProviderError;

impl Display for ParseOAuthAccountProviderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "unrecognized OAuthAccountProvider variant".fmt(f)
    }
}

impl std::error::Error for ParseOAuthAccountProviderError {}

impl FromStr for OAuthAccountProvider {
    type Err = ParseOAuthAccountProviderError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "APPLE" => Ok(Self::APPLE),
            "GITHUB" => Ok(Self::GITHUB),
            "GOOGLE" => Ok(Self::GOOGLE),
            _ => Err(ParseOAuthAccountProviderError),
        }
    }
}

impl Display for OAuthAccountProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (match self {
            Self::APPLE => "APPLE",
            Self::GITHUB => "GITHUB",
            Self::GOOGLE => "GOOGLE",
        })
        .fmt(f)
    }
}

impl<DB> FromSql<Text, DB> for OAuthAccountProvider
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> diesel::deserialize::Result<Self> {
        Ok(String::from_sql(bytes)?.parse()?)
    }
}

impl<DB> ToSql<Text, DB> for OAuthAccountProvider
where
    DB: Backend,
    String: ToSql<Text, DB>,
{
    fn to_sql<W: std::io::Write>(
        &self,
        out: &mut diesel::serialize::Output<W, DB>,
    ) -> diesel::serialize::Result {
        self.to_string().to_sql(out)
    }
}

#[derive(
    Debug, diesel::Associations, diesel::Identifiable, diesel::Queryable, graphql::SimpleObject,
)]
#[belongs_to(User)]
#[graphql(name = "OAuthAccount")]
#[table_name = "oauth_accounts"]
pub struct OAuthAccount {
    #[graphql(skip)]
    pub _rowid: i32,
    pub created_at: DateTime,
    #[graphql(skip)]
    pub deleted_at: Option<DateTime>,
    pub id: uuid::Uuid,
    pub provider: OAuthAccountProvider,
    pub provider_access_token: String,
    pub provider_access_token_expires_at: DateTime,
    pub provider_account_id: String,
    pub provider_refresh_token: String,
    pub updated_at: DateTime,
    #[graphql(skip)]
    pub user_id: uuid::Uuid,
}

#[derive(Debug, diesel::Insertable, graphql::InputObject)]
#[graphql(name = "OAuthAccountCreateInput")]
#[table_name = "oauth_accounts"]
pub struct OAuthAccountCreateInput {
    pub provider: OAuthAccountProvider,
    pub provider_access_token: String,
    pub provider_access_token_expires_at: DateTime,
    pub provider_account_id: String,
    pub provider_refresh_token: String,
    pub user_id: uuid::Uuid,
}

#[derive(Debug, graphql::SimpleObject)]
#[graphql(name = "OAuthAccountCreateOutput")]
pub struct OAuthAccountCreateOutput {
    pub oauth_account: OAuthAccount,
}

#[derive(Debug, graphql::SimpleObject)]
pub struct PageInfo {
    pub has_next_page: bool,
    pub has_previous_page: bool,
}

#[derive(
    Debug, diesel::Associations, diesel::Identifiable, diesel::Queryable, graphql::SimpleObject,
)]
#[belongs_to(Blog)]
#[graphql(complex)]
pub struct Post {
    #[graphql(skip)]
    pub _rowid: i32,
    #[graphql(skip)]
    pub blog_id: uuid::Uuid,
    pub created_at: DateTime,
    #[graphql(skip)]
    pub deleted_at: Option<DateTime>,
    pub id: uuid::Uuid,
    pub slug: String,
    pub updated_at: DateTime,
}

#[graphql::ComplexObject]
impl Post {
    pub async fn blog(&self, ctx: &Context<'_>) -> graphql::Result<Blog> {
        let pool = ctx.data_unchecked::<crate::db::Pool>();

        Ok(blogs::table.find(self.blog_id).get_result(&pool.get()?)?)
    }
}

impl Node for Post {
    fn cursor(&self) -> Cursor {
        Cursor {
            _rowid: self._rowid,
            ty: String::from("Post"),
        }
    }
}

#[derive(Debug, diesel::Insertable, graphql::InputObject)]
#[table_name = "posts"]
pub struct PostCreateInput {
    pub blog_id: uuid::Uuid,
    pub slug: String,
}

#[derive(Debug, graphql::SimpleObject)]
pub struct PostCreateOutput {
    pub post: Post,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, diesel::AsExpression, diesel::FromSqlRow, graphql::Enum,
)]
#[sql_type = "Text"]
pub enum UserRole {
    ADMIN,
    USER,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseUserRoleError;

impl Display for ParseUserRoleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "unrecognized UserRole variant".fmt(f)
    }
}

impl std::error::Error for ParseUserRoleError {}

impl FromStr for UserRole {
    type Err = ParseUserRoleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ADMIN" => Ok(Self::ADMIN),
            "USER" => Ok(Self::USER),
            _ => Err(ParseUserRoleError),
        }
    }
}

impl Display for UserRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (match self {
            Self::ADMIN => "ADMIN",
            Self::USER => "USER",
        })
        .fmt(f)
    }
}

impl<DB> FromSql<Text, DB> for UserRole
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> diesel::deserialize::Result<Self> {
        Ok(String::from_sql(bytes)?.parse()?)
    }
}

impl<DB> ToSql<Text, DB> for UserRole
where
    DB: Backend,
    String: ToSql<Text, DB>,
{
    fn to_sql<W: std::io::Write>(
        &self,
        out: &mut diesel::serialize::Output<W, DB>,
    ) -> diesel::serialize::Result {
        self.to_string().to_sql(out)
    }
}

#[derive(Debug, diesel::Identifiable, diesel::Queryable, graphql::SimpleObject)]
#[graphql(complex)]
pub struct User {
    #[graphql(skip)]
    pub _rowid: i32,
    pub created_at: DateTime,
    #[graphql(skip)]
    pub deleted_at: Option<DateTime>,
    pub id: uuid::Uuid,
    pub role: UserRole,
    pub updated_at: DateTime,
}

#[graphql::ComplexObject]
impl User {
    pub async fn blogs(
        &self,
        ctx: &Context<'_>,
        first: i64,
        after: Option<Cursor>,
    ) -> graphql::Result<Connection<Blog>> {
        let after = after.unwrap_or_default();

        let pool = ctx.data_unchecked::<crate::db::Pool>();
        let nodes: Vec<Blog> = Blog::belonging_to(self)
            .filter(crate::schema::blogs::_rowid.gt(after._rowid))
            .limit(first)
            .get_results(&pool.get()?)?;

        Ok(Connection {
            edges: nodes.into_iter().map(Edge::from).collect(),
            page_info: PageInfo {
                has_next_page: crate::schema::blogs::table
                    .count()
                    .get_result::<i64>(&pool.get()?)?
                    > (after._rowid as i64 + 1),
                has_previous_page: after._rowid > 0,
            },
        })
    }

    pub async fn email_account(&self, ctx: &Context<'_>) -> graphql::Result<Option<EmailAccount>> {
        let pool = ctx.data_unchecked::<crate::db::Pool>();

        Ok(EmailAccount::belonging_to(self)
            .get_result(&pool.get()?)
            .optional()?)
    }

    pub async fn oauth_accounts(&self, ctx: &Context<'_>) -> graphql::Result<Vec<OAuthAccount>> {
        let pool = ctx.data_unchecked::<crate::db::Pool>();

        Ok(OAuthAccount::belonging_to(self)
            .order_by(oauth_accounts::created_at.asc())
            .get_results(&pool.get()?)?)
    }
}

impl Node for User {
    fn cursor(&self) -> Cursor {
        Cursor {
            _rowid: self._rowid,
            ty: String::from("User"),
        }
    }
}

#[derive(Debug, diesel::Insertable, graphql::InputObject)]
#[table_name = "users"]
pub struct UserCreateInput {
    pub role: UserRole,
}

#[derive(Debug, graphql::SimpleObject)]
pub struct UserCreateOutput {
    pub user: User,
}
