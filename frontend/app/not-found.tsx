import React from 'react';

export default function NotFound() {
    return (
        <div style={{ display: 'flex', minHeight: '100vh', alignItems: 'center', justifyContent: 'center', flexDirection: 'column', padding: '0 1rem' }}>
            <h1 style={{ fontSize: '3rem', margin: 0 }}>404</h1>
            <h2 style={{ fontSize: '1.5rem', margin: '0.5rem 0' }}>Page Not Found</h2>
            <p style={{ fontSize: '1rem', textAlign: 'center' }}>
                The page you are looking for does not exist.
            </p>
        </div>
    );
}