import React, { useState } from "react";
import { QueryClient, useQuery } from "react-query";
import { dehydrate, DehydratedState } from "react-query/hydration";
import { getSdk, graphqlClient } from "../sdk";

import type { GetServerSideProps } from "next";
import type { Scalars } from "../sdk";

const FIRST = 10;

export const getServerSideProps: GetServerSideProps<{
    dehydratedState: DehydratedState;
}> = async () => {
    const queryClient = new QueryClient();
    await queryClient.prefetchQuery(["posts", null], () => (
        getSdk(graphqlClient).posts({ after: null, first: FIRST })
    ));

    return { props: { dehydratedState: dehydrate(queryClient) } };
};

const Dashboard = (): JSX.Element | null => {
    const [after] = useState<Scalars["Cursor"] | null>(null);
    const sdk = getSdk(graphqlClient);
    const { data } = useQuery(["posts", after], () => (
        sdk.posts({ after: null, first: FIRST })
    ));

    return (
        <ul>
            {data?.posts.edges.map((post) => (
                <li key={post.node.id}>
                    <pre>
                        {JSON.stringify(post.node, null, 2)}
                    </pre>
                </li>
            ))}
        </ul>
    );
};

export default Dashboard;
