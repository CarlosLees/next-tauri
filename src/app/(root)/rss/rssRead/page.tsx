'use client';

import { useSearchParams } from 'next/navigation';
import { useEffect } from 'react';

const RssRead = () => {
    const searchParams = useSearchParams();
    const link = searchParams.get('link');

    useEffect(() => {
        fetch(link).then((res) => {
            res.json().then((json) => {
                console.log('json:', json);
            });
        });
    }, []);

    return (
        <div>
            {link && (
                <iframe className="w-full h-[100vh]" src={link} title="External Site Preview" />
            )}
        </div>
    );
};

export default RssRead;
