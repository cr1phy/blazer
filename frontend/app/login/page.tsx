"use client"
import { useState } from "react";
import { useRouter } from "next/navigation";

export default function Login() {
    const [email, setEmail] = useState("");
    const [password, setPassword] = useState("");
    const router = useRouter();

    const handleLogin = async () => {
        try {
            const res = await fetch("/api/acc/login", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({ email, password })
            });

            if (!res.ok) {
                const errorData = await res.json();
                throw new Error(errorData.message || "Login failed");
            }

            router.push("/chats");
        } catch (error) {
            alert(error.message);
        }
    };

    return (
        <div className="flex flex-col items-center p-10">
            <input className="p-2 border rounded" value={email} onChange={(e) => setEmail(e.target.value)} placeholder="Email" type="email" required />
            <input className="p-2 border rounded mt-2" type="password" value={password} onChange={(e) => setPassword(e.target.value)} placeholder="Password" required />
            <button className="mt-4 bg-blue-500 text-white p-2 rounded" onClick={handleLogin}>Login</button>
        </div>
    );
}
