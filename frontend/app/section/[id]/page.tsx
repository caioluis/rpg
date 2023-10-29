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

async function getSectionData(id: string): Promise<SectionData> {
    const hanko_cookie = cookies().get('hanko')?.value;
    const res = await fetch(`http://127.0.0.1:3000/core/section/${id}`, {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json',
            'Cookie': `hanko=${hanko_cookie}`
        }
    });

    if (!res.ok) {
        console.error(`Failed to fetch section data. Status: ${res.status}`);
        throw new Error('Failed to fetch section data');
    }

    return await res.json();
}

// TODO: make photo redirect to user profile

export default async function Section({params}: { params: { id: string } }) {
    const data = await getSectionData(params.id);
    return (
        <div className="flex flex-col items-center">
            <h1 className="text-4xl font-bold">{data.section.title}</h1>
            <div className="w-10/12 xl:w-3/5 mt-6">
                <Table>
                    <TableHeader>
                        <TableRow>
                            <TableHead>Título</TableHead>
                            <TableHead>Autor do tópico</TableHead>
                            <TableHead className="text-right">Último post</TableHead>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        {data.topics.map((topicPreview) => (
                            <TableRow key={topicPreview.topic.id}>
                                <TableCell>
                                        <Link href={`/topic/${topicPreview.topic.id}`}>
                                            {topicPreview.topic.title}
                                        </Link>
                                    </TableCell>
                                <TableCell>
                                    <Link href={`/#`}>
                                         Nome do autor
                                    </Link>
                                </TableCell>
                                <TableCell className="flex justify-end items-center gap-2">
                                    <Link href={`/#`}>
                                        <Avatar>
                                            <AvatarImage src="https://placehold.co/100" alt="Dattebayo!" />
                                            <AvatarFallback>D</AvatarFallback>
                                        </Avatar>
                                    </Link>
                                    {/* TODO: make it go directly to the page location the post is*/}
                                    <Link href={`/topic/${topicPreview.topic.id}`}>
                                        <GoTo />
                                    </Link>
                                </TableCell>
                            </TableRow>
                        ))}
                    </TableBody>
                </Table>
            </div>
        </div>
    )
}