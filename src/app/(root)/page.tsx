'use client';

import { invoke } from '@tauri-apps/api/tauri';
import { useState } from 'react';

const Home = () => {
    const [ipAddr, setIpAddr] = useState('');

    const getIp = async () => {
        invoke('local_ip').then((res: unknown) => {
            res && setIpAddr(res as string);
        });
    };

    return (
        <div>
            Home
            <button className="bg-white-1" onClick={getIp}>
                get ip
            </button>
            <div className="bg-white-1">{ipAddr}</div>
        </div>
    );
};

export default Home;
