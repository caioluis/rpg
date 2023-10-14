"use client"

import { useEffect } from "react";
import { register } from "@teamhanko/hanko-elements";

const hankoApi = process.env.NEXT_PUBLIC_HANKO_API_URL;

export default function HankoAuth() {
    useEffect(() => {
        // @ts-ignore
        register(hankoApi)
            .catch((error) => {
                console.error(error);
            });
    }, []);

    return (
        <hanko-auth />
    );
}
