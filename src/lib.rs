#[macro_use]
extern crate diesel;

pub mod db;
pub mod models;
pub mod schema;

use diesel::prelude::*;
use graphql::Context;

use crate::models::{Connection, *};
use crate::schema::*;

pub type Schema = graphql::Schema<QueryRoot, MutationRoot, SubscriptionRoot>;

#[derive(Debug, Default)]
pub struct QueryRoot;

#[graphql::Object]
impl QueryRoot {
    async fn blog(&self, ctx: &Context<'_>, id: uuid::Uuid) -> graphql::Result<Option<Blog>> {
        let pool = ctx.data_unchecked::<crate::db::Pool>();

        Ok(crate::schema::blogs::table
            .find(id)
            .get_result(&pool.get()?)
            .optional()?)
    }

    async fn blogs(
        &self,
        ctx: &Context<'_>,
        first: i64,
        after: Option<Cursor>,
    ) -> graphql::Result<Connection<Blog>> {
        let after = after.unwrap_or_default();

        let pool = ctx.data_unchecked::<crate::db::Pool>();
        let nodes: Vec<Blog> = blogs::table
            .filter(blogs::_rowid.gt(after._rowid))
            .limit(first)
            .get_results(&pool.get()?)?;

        Ok(Connection {
            edges: nodes.into_iter().map(Edge::from).collect(),
            page_info: PageInfo {
                has_next_page: blogs::table.count().get_result::<i64>(&pool.get()?)?
                    > (after._rowid as i64 + 1),
                has_previous_page: after._rowid > 0,
            },
        })
    }

    async fn post(&self, ctx: &Context<'_>, id: uuid::Uuid) -> graphql::Result<Option<Post>> {
        let pool = ctx.data_unchecked::<crate::db::Pool>();

        Ok(posts::table.find(id).get_result(&pool.get()?).optional()?)
    }

    async fn posts(
        &self,
        ctx: &Context<'_>,
        first: i64,
        after: Option<Cursor>,
    ) -> graphql::Result<Connection<Post>> {
        let after = after.unwrap_or_default();

        let pool = ctx.data_unchecked::<crate::db::Pool>();
        let nodes: Vec<Post> = posts::table
            .filter(posts::_rowid.gt(after._rowid))
            .limit(first)
            .get_results(&pool.get()?)?;

        Ok(Connection {
            edges: nodes.into_iter().map(Edge::from).collect(),
            page_info: PageInfo {
                has_next_page: posts::table.count().get_result::<i64>(&pool.get()?)?
                    > (after._rowid as i64 + 1),
                has_previous_page: after._rowid > 0,
            },
        })
    }

    async fn user(&self, ctx: &Context<'_>, id: uuid::Uuid) -> graphql::Result<Option<User>> {
        let pool = ctx.data_unchecked::<crate::db::Pool>();

        Ok(users::table.find(id).get_result(&pool.get()?).optional()?)
    }

    async fn users(
        &self,
        ctx: &Context<'_>,
        first: i64,
        after: Option<Cursor>,
    ) -> graphql::Result<Connection<User>> {
        let after = after.unwrap_or_default();

        let pool = ctx.data_unchecked::<crate::db::Pool>();
        let nodes: Vec<User> = users::table
            .filter(users::_rowid.gt(after._rowid))
            .limit(first)
            .get_results(&pool.get()?)?;

        Ok(Connection {
            edges: nodes.into_iter().map(Edge::from).collect(),
            page_info: PageInfo {
                has_next_page: users::table.count().get_result::<i64>(&pool.get()?)?
                    > (after._rowid as i64 + 1),
                has_previous_page: after._rowid > 0,
            },
        })
    }

    async fn version(&self) -> &'static str {
        concat!("v", env!("CARGO_PKG_VERSION"))
    }
}

#[derive(Debug, Default)]
pub struct MutationRoot;

#[graphql::Object]
impl MutationRoot {
    async fn blog_create(
        &self,
        ctx: &Context<'_>,
        blog: BlogCreateInput,
    ) -> graphql::Result<BlogCreateOutput> {
        let pool = ctx.data_unchecked::<crate::db::Pool>();

        Ok(BlogCreateOutput {
            blog: diesel::insert_into(blogs::table)
                .values(&blog)
                .returning(blogs::all_columns)
                .get_result(&pool.get()?)?,
        })
    }

    async fn email_account_create(
        &self,
        ctx: &Context<'_>,
        email_account: EmailAccountCreateInput,
    ) -> graphql::Result<EmailAccountCreateOutput> {
        let pool = ctx.data_unchecked::<crate::db::Pool>();

        Ok(EmailAccountCreateOutput {
            email_account: diesel::insert_into(email_accounts::table)
                .values(&email_account)
                .returning(email_accounts::all_columns)
                .get_result(&pool.get()?)?,
        })
    }

    async fn oauth_account_create(
        &self,
        ctx: &Context<'_>,
        oauth_account: OAuthAccountCreateInput,
    ) -> graphql::Result<OAuthAccountCreateOutput> {
        let pool = ctx.data_unchecked::<crate::db::Pool>();

        Ok(OAuthAccountCreateOutput {
            oauth_account: diesel::insert_into(oauth_accounts::table)
                .values(&oauth_account)
                .returning(oauth_accounts::all_columns)
                .get_result(&pool.get()?)?,
        })
    }

    async fn post_create(
        &self,
        ctx: &Context<'_>,
        post: PostCreateInput,
    ) -> graphql::Result<PostCreateOutput> {
        let pool = ctx.data_unchecked::<crate::db::Pool>();

        Ok(PostCreateOutput {
            post: diesel::insert_into(posts::table)
                .values(&post)
                .returning(posts::all_columns)
                .get_result(&pool.get()?)?,
        })
    }

    async fn user_create(
        &self,
        ctx: &Context<'_>,
        user: UserCreateInput,
    ) -> graphql::Result<UserCreateOutput> {
        let pool = ctx.data_unchecked::<crate::db::Pool>();

        Ok(UserCreateOutput {
            user: diesel::insert_into(users::table)
                .values(&user)
                .returning(users::all_columns)
                .get_result(&pool.get()?)?,
        })
    }
}

pub type SubscriptionRoot = graphql::EmptySubscription;
