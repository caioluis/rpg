import HankoAuth from "@/components/HankoAuth";
import { Metadata } from "next"
import Image from "next/image"
import Link from "next/link"

import { cn } from "@/lib/utils"

export const metadata: Metadata = {
    title: "Login",
    description: "Faça (login) ou crie uma conta no Dattebayo!",
}

export default function LoginPage() {
        return (
            <div className="container relative h-screen flex-col items-center justify-center grid lg:max-w-none lg:grid-cols-2 lg:px-0 bg-neutral-950">
                <div className="relative hidden h-full flex-col bg-muted p-10 text-white dark:border-r lg:flex bg-neutral-950">
                    <Image
                        src="/register/background.png"
                        fill={true}
                        alt="Authentication"
                        className="absolute inset-0 w-full h-full object-cover"
                        draggable={false}
                    />
                    <div className="absolute self-center w-7/12 2xl:w-9/12 bottom-0 z-[3]">
                        <Image src="/register/naruto.png" sizes="100vw"
                           style={{
                               width: '100%',
                               height: 'auto',
                           }}
                           width={928}
                           height={1080}
                           alt="Naruto"
                           draggable={false}

                        />
                    </div>
                    <div className="relative z-20 flex flex-col items-center text-lg font-medium mt-4 3xl:mt-12">
                        <div className="relative h-[35px] w-full lg:h-[80px] flex items-center 3xl:h-[140px]">
                            <Image src="/DattebayoNameLogo.svg" alt="Dattebayo!" fill={true} />
                        </div>
                        <div className="relative my-12">
                            <h1 className="text-5xl text-center text-white font-extrabold mb-4 xl:text-7xl">Olá, Genin!</h1>
                            <p className="text-center font-semibold xl:text-3xl">
                                Seja bem-vindo ao <em>Dattebayo!</em>, O RPG de Naruto mais completo da internet.
                            </p>
                        </div>
                    </div>
                </div>
                <div className="lg:p-8 bg-neutral-950">
                    <div className="mx-auto flex w-full flex-col justify-center space-y-6 sm:w-[350px]">
                        <div className="flex flex-col space-y-2 items-center text-center">
                            <Image src="/DattebayoLogo.svg" sizes="100vw"
                                   style={{
                                       width: 100,
                                       height: 'auto',
                                   }}
                                   width={200}
                                   height={200}
                                   alt="Dattebayo!"
                                   draggable={false}
                            />
                            <h1 className="text-2xl text-white font-semibold tracking-tight">
                                Cadastre-se!
                            </h1>
                            <p className="text-sm text-white text-muted-foreground">
                                Insira seu e-mail abaixo para criar uma conta ou faça login com a sua chave de acesso.
                            </p>
                        </div>
                        <HankoAuth />
                    </div>
                </div>
            </div>
        );
}
