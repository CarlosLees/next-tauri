'use client';

import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import Image from 'next/image';

import { useToast } from '@/components/ui/use-toast';

const TopBar = () => {
    const { toast } = useToast();

    const [ipAddr, setIpAddr] = useState('');

    useEffect(() => {
        getIp().then();
    }, []);

    const getIp = async () => {
        invoke('local_ip').then((res: unknown) => {
            res && setIpAddr(res as string);
        });
    };

    const copyIp = async () => {
        ipAddr && (await navigator.clipboard.writeText(ipAddr));
        toast({
            description: '123',
        });
    };

    return (
        <div className="flex justify-end items-center h-10 text-white-1">
            <div className="flex gap-2">
                <div className="font-bold">{ipAddr}</div>
                <Image
                    className="cursor-pointer"
                    src="/icons/copy.svg"
                    alt="复制"
                    width={20}
                    height={20}
                    onClick={copyIp}
                />
            </div>
        </div>
    );
};

export default TopBar;
