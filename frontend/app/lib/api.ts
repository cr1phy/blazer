import axios from "axios";

export async function fetchMessages(id: string): Promise<Message[]> {
    const res = await axios.get(`/api/chat/${id}`);
    if (res.status !== 200) throw new Error("Failed to fetch messages");
    return await res.request.json();
}