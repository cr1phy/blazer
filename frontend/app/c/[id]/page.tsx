"use client"

import { useEffect, useState } from "react";

interface Message {
    text: string;
}

export default async function Chat({ params }: { params: Promise<{ id: string }> }) {
    const id = (await params).id;
    const [messages, setMessages] = useState<Message[] | null>();

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
                return;
            }
        };
        fetchMessages();
    }, [id]);

    return (
        <div className="flex flex-col p-10">
            <h1 className="text-2xl font-bold">Chat {id}</h1>
            <div className="p-2 border rounded mt-4">
                {messages == null && messages!!.length ? messages!!.map((msg, index) => (
                    <p key={index}>{msg.text}</p>
                )) : "No messages yet."}
            </div>
        </div>
    );
}
