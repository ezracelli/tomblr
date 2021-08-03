import React from "react";
import Error, { ErrorProps } from "next/error";

const Error_ = (props: ErrorProps): JSX.Element | null => (
    <Error
        {...props}
        statusCode={props.statusCode ?? 404}
    />
);

export default Error_;
