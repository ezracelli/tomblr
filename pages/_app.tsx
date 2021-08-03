import "tailwindcss/tailwind.css";

import React from "react";
import { QueryClient, QueryClientProvider } from "react-query";
import { Hydrate } from "react-query/hydration";

import type { AppProps } from "next/app";

const queryClient = new QueryClient();

const App = ({
    Component,
    pageProps: {
        dehydratedState,
        ...pageProps
    },
}: AppProps): JSX.Element | null => (
    <QueryClientProvider client={queryClient}>
        <Hydrate state={dehydratedState}>
            <div className="bg-white dark:bg-gray-800 h-screen w-screen">
                <Component {...pageProps} />
            </div>
        </Hydrate>
    </QueryClientProvider>
);

export default App;
