'use client';

import { useEffect, useState } from 'react';

import { invoke } from '@tauri-apps/api/tauri';

import PodcastCard from '@/components/podcastCard/PodcastCard';

interface AppModel {
    name: string;
    icon: string;
}

export default function Home() {
    const [appList, setAppList] = useState<AppModel[]>([]);

    useEffect(() => {
        invoke('app_list').then((res) => {
            setAppList(res as AppModel[]);
        });
    }, []);

    return (
        <div className="mt-9 flex flex-col gap-9">
            <section className="flex flex-col gap-5">
                {/* <h1 className="text-20 font-bold text-white-1">Trending Podcast</h1> */}

                <div className="podcast_grid">
                    {appList.map(({ name, icon }) => {
                        return <PodcastCard key={name} title={name} imageURL={icon} />;
                    })}
                </div>
            </section>
        </div>
    );
}
