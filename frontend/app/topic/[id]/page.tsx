import { cookies } from 'next/headers'

import {
    Table,
    TableBody,
    TableCaption,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
} from "@/components/ui/table"
import {Avatar, AvatarImage, AvatarFallback} from "@/components/ui/avatar";
import {GoTo} from "@/components/navigation/GoTo";
import Link from "next/link";
import {Button} from "@/components/Button";

interface SectionData {
    section: Section;
    topics: TopicPreview[]
    children_sections: Section[];
}

interface Topic {
    id: string; // Uuid
    created_by: string; // Uuid
    updated_by?: string; // Optional Uuid
    section_id: string; // Uuid
    locked: boolean;
    title: string;
    created_at: Date; // DateTime<Utc>
    updated_at?: Date; // Optional DateTime<Utc>
}

interface MostRecentPost {
    id: string; // Uuid
    created_by: string; // Uuid
    created_at: Date; // DateTime<Utc>
    updated_by?: string; // Optional Uuid
}

interface TopicPreview {
    topic: Topic;
    most_recent_post: MostRecentPost;
}

interface Section {
    id: string; // Uuid
    parent_section_id?: string; // Optional Uuid
    updated_by?: string; // Optional Uuid
    title: string;
    description: string;
    locked: boolean;
    created_at: Date; // DateTime<Utc>
    updated_at?: Date; // Optional DateTime<Utc>
}
export default async function Section({params}: { params: { id: string } }) {
    return (
            <div className="max-w-sm mx-auto bg-neutral-800 rounded-xl shadow-lg overflow-hidden md:max-w-2xl transform transition duration-500 hover:scale-105">
                <div className="md:flex">
                    <div className="md:flex-shrink-0">
                        {/* TODO: make the round-y profile picture so it can be 100x100 little circle in lower resolutions*/}
                        <img
                            alt="Profile Picture"
                            className="h-48 w-full object-cover md:h-full md:w-48"
                            height="160"
                            src="https://placehold.co/90x160"
                            style={{
                                aspectRatio: "9/16",
                                objectFit: "cover",
                            }}
                            width="48"
                        />
                    </div>
                    <div className="p-8">
                        {/* TODO: perhaps apply the color of the user's group to the name instead*/}
                        <div className="uppercase tracking-wide text-sm text-neutral-400 font-bold">Nome do autor do post</div>
                        <p className="mt-2 text-neutral-300">
                            Lorem ipsum dolor sit amet, consectetur adipiscing elit. Molestie parturient et sem ipsum volutpat vel. Natoque sem et aliquam mauris egestas quam volutpat viverra. In pretium nec senectus erat. Et malesuada lobortis.
                        </p>
                    </div>
                </div>
                <div className="border-t-2 border-gray-200 border-neutral-700 p-2 bg-neutral-800">
                    <div className="text-neutral-400 m-0.5">
                        Alguma informação sobre o post
                    </div>
                </div>
            </div>
    )
}