"use client";

import { useState, useEffect } from "react";
import { useRouter } from "next/navigation";
import { Hanko } from "@teamhanko/hanko-elements";
import {cn} from "@/lib/utils";

const hankoApi = process.env.NEXT_PUBLIC_HANKO_API_URL;

export function LogoutBtn() {
    const router = useRouter();
    const [hanko, setHanko] = useState<Hanko>();

    useEffect(() => {
        import("@teamhanko/hanko-elements").then(({ Hanko }) =>
            setHanko(new Hanko(hankoApi ?? ""))
        );
    }, []);

    const logout = async () => {
        try {
            await hanko?.user.logout();
            router.push("/login");
            router.refresh();
            return;
        } catch (error) {
            console.error("Error during logout:", error);
        }
    };

    return <button onClick={logout} className="w-full block px-4 py-2 text-sm text-neutral-200 hover:bg-neutral-800 text-left">Logout</button>;
}
