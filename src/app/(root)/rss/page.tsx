'use client';

import { invoke } from '@tauri-apps/api/tauri';
import { useState } from 'react';

interface RssChannel {
    title: string;
    link: string;
    description: string;
    pubDate: string;
    items: RssChannelItem[];
}

interface RssChannelItem {
    title: string;
    link: string;
    author: string;
    description: string;
    category: string;
    guid: string;
    pubDate: string;
}

const Rss = () => {
    const [rss, setRss] = useState<RssChannel>();

    const getRss = async () => {
        const rssList = (await invoke('rss_list')) as RssChannel;
        console.log(rssList);
        setRss(rssList);
    };

    return (
        <div>
            {!rss ||
                (rss.items &&
                    rss.items.map((channel) => {
                        return (
                            <div className="text-white-1" key={channel.title}>
                                {channel.link}
                            </div>
                        );
                    }))}
            <button className="bg-white-1" onClick={getRss}>
                获取rss
            </button>
        </div>
    );
};

export default Rss;
