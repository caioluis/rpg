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

interface SectionData {
    section: Section;
    topics: Topic[]
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

export default async function Section({params}: { params: { id: string } }) {
    const data = await getSectionData(params.id);
    return (
        <Table>
            <TableCaption>A list of your recent invoices.</TableCaption>
            <TableHeader>
                <TableRow>
                    <TableHead>Título</TableHead>
                    <TableHead>Autor do tópico</TableHead>
                </TableRow>
            </TableHeader>
            <TableBody>
                {data.topics.map((topic) => (
                    <TableRow key={topic.id}>
                        <TableCell>{topic.title}</TableCell>
                        <TableCell>{topic.created_by}</TableCell>
                    </TableRow>
                ))}
            </TableBody>
        </Table>
    )
}