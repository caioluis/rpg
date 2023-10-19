import Image from "next/image";
import {Button} from "@/components/Button";
import Link from "next/link";

export const RegrasETutoriais = () => {
    return (
        <div className="flex justify-between">
            <div className="relative mt-10 h-[180px] xs:h-[260px] s:h-[300px] sm:h-[330px] md:h-[370px] mmd:h-[400px] p-4 xs:p-6 rounded-md z-[1] lg:h-[420px] w-8/12 bg-neutral-950 bg-clip-padding backdrop-filter backdrop-blur-sm bg-opacity-70 border border-neutral-800">
                <div className="absolute h-[190px] w-[220px] xs:h-[260px] xs:w-[290px] s:h-[320px] s:w-[350px] sm:h-[350px] sm:w-[380px] md:h-[390px] md:w-[420px] mmd:h-[420px] mmd:w-[450px] lg:h-[440px] lg:w-[470px] z-[3] left-[110px] xs:left-[90px] s:left-[140px] sm:left-[170px] md:left-[240px] bottom-0">
                    <Image src="https://i.imgur.com/LRhF1aI.png" fill={true} alt="Hashirama" />
                </div>
                <h2 className="pb-0.25 sm:pb-2 text-xs sm:text-base font-bold text-neutral-100 sm:text-2xl sm:leading-none sm:tracking-tight lg:text-2xl">
                    Regras e Tutorias
                </h2>
                <h3 className="text-orange-400 text-[9px] sm:text-base pb-1">X Postagens • X Tópicos</h3>
                <p className="relative z-[4] text-[8px] sm:text-base font-medium text-neutral-100 drop-shadow-[0_2px_2px_rgba(0,0,0,1)]">
                    Conheça todas as regras e informações úteis para a sua jornada como um grande ninja! Um bom conhecimento das
                    regras é vital para o bem-estar do RPG.
                </p>
                <h2 className="pt-2 text-xs hidden sm:inline-block sm:sm:text-base font-bold text-neutral-100 sm:text-2xl sm:leading-none sm:tracking-tight lg:text-2xl">
                    Regras Iniciais
                </h2>
                <div className="relative text-[7px] sm:text-base pt-2 z-[4] hidden sm:grid grid-rows-2 gap-2">
                    <div className="flex gap-1">
                        <Button asChild>
                            <Link href="/regras-gerais">Regras Gerais</Link>
                        </Button>
                        <Button>
                            <Link href="/missoes">Missões</Link>
                        </Button>
                        <Button>
                            <Link href="/roleplays">Roleplays</Link>
                        </Button>
                        <Button>
                            <Link href="/combates">Combates</Link>
                        </Button>
                    </div>
                    <div className="flex gap-2">
                        <Button>
                            <Link href="/nukenins">Nukenins</Link>
                        </Button>
                        <Button>
                            <Link href="/habilidades-base">Habilidades Base</Link>
                        </Button>
                        <Button>
                            <Link href="/danos-e-potencia">Danos e Potência</Link>
                        </Button>
                    </div>
                </div>
                <Button className="absolute text-[10px] h-6 bottom-2 sm:hidden" asChild>
                    <Link href="/regras-tutoriais">Visitar seção das regras</Link>
                </Button>
            </div>
            <div className="relative mt-10 h-[180px] xs:h-[260px] s:h-[300px] sm:h-[330px] md:h-[370px] mmd:h-[400px] p-2 xs:p-6 rounded-md z-[1] lg:h-[420px] w-[30%] bg-neutral-950 bg-clip-padding backdrop-filter backdrop-blur-sm bg-opacity-70 border border-neutral-800">
                <h2 className="pb-2 text-xs sm:text-base font-bold text-neutral-100 sm:text-2xl sm:leading-none sm:tracking-tight lg:text-2xl">
                    Atualizações
                </h2>
            </div>
        </div>
    )
}