'use client';

import { invoke } from '@tauri-apps/api/tauri';
import { useState } from 'react';
import { useRouter } from 'next/navigation';

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
    const router = useRouter();

    const [rss, setRss] = useState<RssChannel>();

    const getRss = async () => {
        const rssList = (await invoke('rss_list')) as RssChannel;
        console.log(rssList);
        setRss(rssList);
    };

    const readRss = (title: string) => {
        console.log('title:', title);
        const channelItems = rss?.items;
        if (channelItems) {
            const find = channelItems.find((item) => item.title === title);
            if (find) {
                router.push(`/rss/rssRead?link=${find.link}`);
            }
        }
    };

    return (
        <div>
            {!rss ||
                (rss.items &&
                    rss.items.map((channel) => {
                        return (
                            <div className="text-white-1" key={channel.title}>
                                {channel.link}
                                <button onClick={() => readRss(channel.title)}>查看</button>
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
