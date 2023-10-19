import './globals.css'
import type {Metadata} from 'next'
import {Inter} from 'next/font/google'
import {cn} from "@/lib/utils";
import Image from 'next/image'
import Navbar from "@/components/navigation/Navbar";


const inter = Inter({subsets: ['latin']})

export const metadata: Metadata = {
    title: 'Dattebayo!',
    description: 'O RPG de Naruto mais completo da internet.',
}


export default function RootLayout({
                                       children,
                                   }: {
    children: React.ReactNode
}) {
    return (
        <html lang="en">
            <body className={cn(inter.className, "dark")}>{children}</body>
        </html>
    )
}
