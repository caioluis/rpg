'use client'

import {Button} from './Button'
import {useEditor, EditorContent} from '@tiptap/react'
import {Paragraph} from "@tiptap/extension-paragraph";
import {Underline} from '@tiptap/extension-underline'
import {BulletList} from "@tiptap/extension-bullet-list";
import StarterKit from '@tiptap/starter-kit'
import {cn} from "@/lib/utils";
import {router} from "next/client";

interface PostInfo {
    created_by: string;
    topic_id: string;
}

async function SendPost(topicId: string, content: string, cookie: string) {
    const res = await fetch(`${process.env.API_URL}/core/topic/${topicId}`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
            'Cookie': `hanko=${cookie}`
        },
        body: content
    });
}

const Tiptap = ({topicId, cookie}: { topicId: string, cookie: string }) => {

    const editor = useEditor({
        extensions: [
            StarterKit,
            Paragraph.configure({
                HTMLAttributes: {
                    class: 'text-neutral-300',
                },
            }),
            BulletList.configure(
                {
                    HTMLAttributes: {
                        class: 'list-disc ml-6',
                    },
                }
            ),
            Underline,
        ],
        content: '<p>Inicie o seu post!</p>',

        editorProps: {
            attributes: {
                class: 'p-4 focus:outline-none',
            },
        }
    })

    if (!editor) {
        return null
    }

    return (
        <>
            <div className="flex space-x-2 p-2 border-b border-neutral-200 w-full">
                <Button className={cn("text-neutral-400", editor.isActive('bold') ? "bg-neutral-950" : "")}
                        variant="secondary" onClick={
                    () => {
                        editor.chain().focus().toggleBold().run()
                    }
                }>
                    <span className="sr-only">Bold</span>
                    <svg
                        className="w-4 h-4"
                        fill="none"
                        height="24"
                        stroke="currentColor"
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth="2"
                        viewBox="0 0 24 24"
                        width="24"
                        xmlns="http://www.w3.org/2000/svg"
                    >
                        <path d="M14 12a4 4 0 0 0 0-8H6v8"/>
                        <path d="M15 20a4 4 0 0 0 0-8H6v8Z"/>
                    </svg>
                </Button>
                <Button className={cn("text-neutral-400", editor.isActive('italic') ? "bg-neutral-950" : "")}
                        variant="secondary" onClick={
                    () => {
                        editor.chain().focus().toggleItalic().run()
                    }
                }>
                    <span className="sr-only">Italic</span>
                    <svg
                        className="w-4 h-4"
                        fill="none"
                        height="24"
                        stroke="currentColor"
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth="2"
                        viewBox="0 0 24 24"
                        width="24"
                        xmlns="http://www.w3.org/2000/svg"
                    >
                        <line x1="19" x2="10" y1="4" y2="4"/>
                        <line x1="14" x2="5" y1="20" y2="20"/>
                        <line x1="15" x2="9" y1="4" y2="20"/>
                    </svg>
                </Button>
                <Button className={cn("text-neutral-400", editor.isActive('underline') ? "bg-neutral-950" : "")}
                        variant="secondary" onClick={
                    () => {
                        editor.chain().focus().toggleUnderline().run()
                    }
                }>
                    <span className="sr-only">Underline</span>
                    <svg
                        className="w-4 h-4"
                        fill="none"
                        height="24"
                        stroke="currentColor"
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth="2"
                        viewBox="0 0 24 24"
                        width="24"
                        xmlns="http://www.w3.org/2000/svg"
                    >
                        <path d="M6 4v6a6 6 0 0 0 12 0V4"/>
                        <line x1="4" x2="20" y1="20" y2="20"/>
                    </svg>
                </Button>
                <Button className={cn("text-neutral-400", editor.isActive('bulletList') ? "bg-neutral-950" : "")}
                        variant="secondary" onClick={
                    () => {
                        editor.chain().focus().toggleBulletList().run()
                    }
                }>
                    <span className="sr-only">Bulleted List</span>
                    <svg
                        className="w-4 h-4"
                        fill="none"
                        height="24"
                        stroke="currentColor"
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth="2"
                        viewBox="0 0 24 24"
                        width="24"
                        xmlns="http://www.w3.org/2000/svg"
                    >
                        <line x1="8" x2="21" y1="6" y2="6"/>
                        <line x1="8" x2="21" y1="12" y2="12"/>
                        <line x1="8" x2="21" y1="18" y2="18"/>
                        <line x1="3" x2="3.01" y1="6" y2="6"/>
                        <line x1="3" x2="3.01" y1="12" y2="12"/>
                        <line x1="3" x2="3.01" y1="18" y2="18"/>
                    </svg>
                </Button>
            </div>
            <EditorContent editor={editor}/>
            <div className="flex flex-col w-full border-t-2 border-t-neutral-700">
                <Button
                    variant={"secondary"}
                    className="w-fit self-end"
                    disabled={editor.getHTML().length === 0}
                    onClick={() => {
                        SendPost(topicId, editor.getHTML(), cookie)
                            .then(() => {
                                router.push(`/topic/${topicId}`)
                            }).catch((err) => {
                                // TODO: this is ugly but it will do for now. Let's change this later
                                console.error(err)
                            }
                        );
                    }
                }>Enviar</Button>
            </div>
        </>
    )
}

export default Tiptap