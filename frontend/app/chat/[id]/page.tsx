"use client"
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";

interface Message {
    text: string;
}

export default function Chat() {
    const router = useRouter();
    const { id } = router.query;
    const [messages, setMessages] = useState<Message[]>([]);

    useEffect(() => {
        if (!id) return;
        const fetchMessages = async () => {
            try {
                const res = await fetch(`/api/chat/${id}`);
                if (!res.ok) throw new Error("Failed to fetch messages");
                const data: Message[] = await res.json();
                setMessages(data);
            } catch (error) {
                console.error(error);
            }
        };
        fetchMessages();
    }, [id]);

    return (
        <div className="flex flex-col p-10">
            <h1 className="text-2xl font-bold">Chat {id}</h1>
            <div className="p-2 border rounded mt-4">
                {messages.length ? messages.map((msg, index) => (
                    <p key={index}>{msg.text}</p>
                )) : "No messages yet."}
            </div>
        </div>
    );
}
