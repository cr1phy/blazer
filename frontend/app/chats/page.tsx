import Link from "next/link";
import { useEffect, useState } from "react";

interface Chat {
    id: string;
}

export default function Chats() {
    const [chats, setChats] = useState<Chat[]>([]);

    useEffect(() => {
        const fetchChats = async () => {
            try {
                const res = await fetch("/api/chats");
                if (!res.ok) throw new Error("Failed to fetch chats");
                const data: Chat[] = await res.json();
                setChats(data);
            } catch (error) {
                console.error(error);
            }
        };
        fetchChats();
    }, []);

    return (
        <div className="flex flex-col p-10">
            <h1 className="text-2xl font-bold">Chats</h1>
            {chats.map((chat) => (
                <Link key={chat.id} href={`/chat/${chat.id}`} className="mt-4 p-2 border rounded">
                    Chat {chat.id}
                </Link>
            ))}
        </div>
    );
}
