import Image from 'next/image'
import Link from 'next/link'
import { Button } from "@/components/Button";

import { Mural, RegrasETutoriais} from "@/components/pageSections";

export default function Home() {
    return (
        <div className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
            <div className="mx-auto max-w-3xl">
                <Mural />
                <RegrasETutoriais />
            </div>
        </div>
    )
}
