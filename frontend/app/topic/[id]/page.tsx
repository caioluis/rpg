"use client"
import Image from "next/image";

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
export default function Topic({params}: { params: { id: string } }) {
    return (
        <>
            <div className="max-w-sm mx-auto bg-neutral-800 rounded-xl shadow-lg overflow-hidden md:max-w-2xl transform transition duration-500 hover:scale-105">
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
                    <div className="p-8">
                        {/* TODO: perhaps apply the color of the user's group to the name instead */}
                        <h2 className="uppercase tracking-wide text-sm text-neutral-400 font-bold">Nome do autor do post</h2>
                        <p className="max-h-64 text-neutral-300 overflow-y-scroll">
                            Lorem ipsum dolor sit amet, consectetur adipiscing elit. Molestie parturient et sem ipsum volutpat vel. Natoque sem et aliquam mauris egestas quam volutpat viverra. In pretium nec senectus erat. Et malesuada lobortis.
                            Lorem ipsum dolor sit amet, consectetur adipiscing elit. Molestie parturient et sem ipsum volutpat vel. Natoque sem et aliquam mauris egestas quam volutpat viverra. In pretium nec senectus erat. Et malesuada lobortis.
                            Lorem ipsum dolor sit amet, consectetur adipiscing elit. Molestie parturient et sem ipsum volutpat vel. Natoque sem et aliquam mauris egestas quam volutpat viverra. In pretium nec senectus erat. Et malesuada lobortis.
                            Lorem ipsum dolor sit amet, consectetur adipiscing elit. Molestie parturient et sem ipsum volutpat vel. Natoque sem et aliquam mauris egestas quam volutpat viverra. In pretium nec senectus erat. Et malesuada lobortis.
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
        </>
    )
}