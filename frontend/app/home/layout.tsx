import './../globals.css'
import type {Metadata} from 'next'
import {Inter} from 'next/font/google'
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
        <>
            <Navbar/>
            <div className="fixed z-[-1] h-screen w-screen">
                <Image
                    src="https://i.imgur.com/hkj0kMn.jpg"
                    fill={true}
                    className="aspect-w-16 aspect-h-9 object-cover"
                    alt="Background"
                    style={{
                        WebkitMaskImage:
                            "-webkit-gradient(linear, left top, left bottom, from(rgba(0,0,0,0)), to(rgba(0,0,0,0.2)))"
                    }}
                />
            </div>
            <main className="mt-24">
                {children}
            </main>
        </>
    )
}
