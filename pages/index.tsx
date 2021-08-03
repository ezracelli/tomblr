import React from "react";
import Link from "next/link";

const Index = (): JSX.Element | null => (
    <div className="flex flex-col gap-4 h-full mx-auto p-4 max-w-screen-2xl">
        <header className="flex gap-3 items-center">
            <h1 className="bg-white dark:bg-black h-10 w-10 rounded-md"></h1>
            <form onSubmit={(e) => void e.preventDefault()}>
                <input
                    autoComplete="false"
                    className="
                        bg-gray-200
                        focus:bg-white
                        bg-opacity-30
                        focus:bg-opacity-100
                        duration-100
                        px-4
                        py-2
                        rounded-sm
                        transition-all
                    "
                    placeholder="Search"
                    type="search"
                />
            </form>

            <div className="flex gap-3 ml-auto">
                <Link href="/login">
                    <a className="bg-green-500 px-4 py-1.5 rounded-sm">
                        Log In
                    </a>
                </Link>
                <Link href="/register">
                    <a className="bg-blue-500 px-4 py-1.5 rounded-sm">
                        Sign Up
                    </a>
                </Link>
            </div>
        </header>

        <main
            className="
                flex
                flex-col
                flex-grow
                gap-6
                items-center
                justify-center
                mx-auto
                w-72
                max-w-full
            "
        >
            <p className="text-center text-white text-xl">
                Make stuff, look at stuff, talk about stuff, find your people.
            </p>
            <div className="flex flex-col gap-4 w-full">
                <Link href="/register">
                    <a className="bg-blue-500 px-4 py-2 rounded-sm text-center w-full">
                        Sign Up
                    </a>
                </Link>
                <Link href="/login">
                    <a className="bg-green-500 px-4 py-2 rounded-sm text-center w-full">
                        Log In
                    </a>
                </Link>
            </div>
        </main>

        <footer className="flex gap-3 dark:text-white text-sm">
            <Link href="/terms">Terms</Link>
            <Link href="/privacy">Privacy</Link>
            <Link href="/jobs">Jobs</Link>
            <Link href="/support">Support</Link>
        </footer>
    </div>
);

export default Index;
