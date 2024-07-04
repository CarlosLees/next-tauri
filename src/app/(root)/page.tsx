'use client';

import { useEffect, useState } from 'react';

import { invoke } from '@tauri-apps/api/tauri';

import PodcastCard from '@/components/podcastCard/PodcastCard';

interface AppModel {
    name: string;
    icon: string;
    icon_base: [];
    url: string;
}

export default function Home() {
    const [appList, setAppList] = useState<AppModel[]>([]);

    useEffect(() => {
        invoke('app_list').then((res) => {
            const dataList = res as AppModel[];
            dataList &&
                dataList.forEach((data) => {
                    data.url = window.URL.createObjectURL(new Blob(data.icon_base));
                    console.log('data.url:', data.url);
                });
            setAppList(res as AppModel[]);
        });
    }, []);

    return (
        <div className="mt-9 flex flex-col gap-9">
            <section className="flex flex-col gap-5">
                {/* <h1 className="text-20 font-bold text-white-1">Trending Podcast</h1> */}

                <div className="podcast_grid">
                    {appList &&
                        appList.map(({ name, url }) => {
                            return <PodcastCard key={name} title={name} url={url} />;
                        })}
                </div>
            </section>
        </div>
    );
}
