'use client';

import { useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

const Splashscreen = () => {
    useEffect(() => {
        invoke('close_splashscreen').then();
    });

    return <div className="text-white-1">splashscreen</div>;
};

export default Splashscreen;
