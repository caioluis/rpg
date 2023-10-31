import Image from "next/image";
import Tiptap from "@/components/Tiptap";
import { cookies } from "next/headers";
import Link from "next/link";


interface TopicData {
    topic: Topic;
    posts: Post[];
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

interface Post {
    id: string; // Uuid
    created_by: string; // Uuid
    created_by_username: string;
    updated_by?: string; // Optional Uuid
    updated_by_username?: string; // Optional Uuid
    topic_id: string; // Uuid
    content: string;
    created_at: Date; // DateTime<Utc>
    updated_at?: Date; // Optional DateTime<Utc>
}

async function getTopicData(id: string, cookie: string): Promise<TopicData> {
    const res = await fetch(`${process.env.API_URL}/core/topic/${id}`, {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json',
            'Cookie': `hanko=${cookie}`
        }
    });

    if (!res.ok) {
        console.error(`Failed to fetch topic data. Status: ${res.status}`);
        throw new Error('Failed to fetch topic data');
    }

    return await res.json();
}

export default async function Topic({params}: { params: { id: string } }) {
    // TODO: handle this better later on x.x
    const hanko_cookie = cookies().get('hanko')?.value || "";
    const data = await getTopicData(params.id, hanko_cookie);

    // TODO: I can move this to a comnmon lib later
    const timeFormatter = new Intl.DateTimeFormat('pt-BR', {
        timeStyle: "medium",
        dateStyle: "short",
    });
     
    return (
        <div className="flex flex-col">
            <h1 className="text-2xl font-bold text-neutral-200 self-center">{data.topic.title}</h1>
            {data.posts.map((post) => (
                <>
                    <div className="mt-6 max-w-xs mx-auto bg-neutral-800 rounded-xl shadow-lg overflow-hidden md:max-w-[800px] transform transition duration-500 hover:scale-105">
                        <div className="md:flex">
                            <div className="md:flex-shrink-0">
                                {/* TODO: maybe make it the vertical 16/9 banner in mobile */}
                                <Image
                                    alt="Profile Picture"
                                    width={192}
                                    height={341}
                                    src="https://placehold.co/192x341.png"
                                />
                            </div>
                            <div className="p-8 w-full">
                                {/* TODO: perhaps apply the color of the user's group to the name instead */}
                                <Link href={`/user/${post.created_by}`}>
                                    <h2 className="uppercase tracking-wide text-lg text-neutral-400 font-bold">{post.created_by_username}</h2>
                                </Link>
                                {post.content}
                            </div>
                        </div>
                        <div className="border-t-2 border-neutral-700 p-2 bg-neutral-800">
                            <div className="text-neutral-400 m-0.5">
                                {timeFormatter.format(new Date(post.created_at))}
                            </div>
                        </div>
                    </div>
                </>
            ))}
            <div className="flex flex-col mt-4 max-w-xs mx-auto bg-neutral-800 rounded-xl shadow-lg overflow-hidden w-11/12 md:max-w-[800px]">
                <Tiptap topicId={data.topic.id} cookie={hanko_cookie} />
            </div>
        </div>
    )
}