import Image from "next/image";

export const Mural = () => {
    return (
        <div className="flex justify-between m-auto mt-10 h-[180px] xs:h-[260px] s:h-[300px] sm:h-[330px] md:h-[370px] mmd:h-[400px] p-2 xs:p-6 rounded-md z-[1] lg:h-[420px] bg-orange-700 bg-clip-padding backdrop-filter backdrop-blur-sm bg-opacity-10 border border-neutral-800">
            <div className="absolute h-[200px] w-[180px] xs:h-[280px] xs:w-[260px] s:h-[320px] s:w-[300px] sm:h-[350px] sm:w-[330px] md:h-[390px] md:w-[370px] mmd:h-[420px] mmd:w-[400px] lg:h-[440px] lg:w-[420px]  bottom-0  z-[3]  left-[70px] xs:left-[90px] s:left-[140px] sm:left-[170px] md:left-[240px]">
                <Image src="https://i.imgur.com/HkfIKQ1.png" fill={true} alt="Temari" />
            </div>
            <div className="w-[80px] xs:w-[100px] s:w-[160px] sm:w-[190px] md:w-[280px] mmd:w-[300px] h-full">
                <h2 className="pb-2 text-lg font-bold text-neutral-100 sm:text-2xl sm:leading-none sm:tracking-tight lg:text-2xl">
                    Mural
                </h2>
                <div className="h-full pb-12 xs:pb-10 sm:pb-9 mmd:pb-8 grid gap-3 grid-cols-1 s:grid-cols-2">
                    <div
                        className="rounded-md bg-neutral-400  z-[4]"
                        style={{
                            backgroundImage: `url("https://i.imgur.com/tojPoN3.png")`,
                            backgroundPosition: "center",
                            backgroundSize: "cover"
                        }}
                    ></div>
                    <div
                        className="rounded-md bg-neutral-400  z-[2]"
                        style={{
                            backgroundImage: `url("https://i.imgur.com/tojPoN3.png")`,
                            backgroundPosition: "center",
                            backgroundSize: "cover"
                        }}
                    ></div>
                    <div
                        className="rounded-md bg-neutral-400 s:col-span-2  z-[4]"
                        style={{
                            backgroundImage: `url("https://i.imgur.com/tojPoN3.png")`,
                            backgroundPosition: "center",
                            backgroundSize: "cover"
                        }}
                    ></div>
                </div>
            </div>
            <div>
                <h2 className="pb-2 text-lg font-bold text-neutral-100 sm:text-2xl sm:leading-none sm:tracking-tight lg:text-2xl">
                    Tópicos Recentes
                </h2>
            </div>
        </div>
    )
}