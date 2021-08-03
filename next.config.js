module.exports = {
    async rewrites () {
        return [{ destination: "http://localhost:4000", source: "/api/graphql.json" }];
    },
};
