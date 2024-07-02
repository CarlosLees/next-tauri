'use client';

import Image from 'next/image';

const imageLoader = ({ src, width }: { src: string; width: number }) => {
    return `${src}?w=${width}`;
};

const PodcastCard = ({ title, imageURL }: { title: string; imageURL: string }) => {
    return (
        <div className="cursor-pointer">
            <figure className="flex flex-col gap-2">
                <Image
                    loader={imageLoader}
                    className="aspect-square h-fit w-full rounded-2xl 2xl:size-[200px]"
                    src={imageURL}
                    alt={title}
                    width={174}
                    height={174}
                />
                <div className="flex flex-col">
                    <h1 className="text-16 truncate font-bold text-white-1">{title}</h1>
                    {/* <h2 className="text-12 truncate font-normal capitalize text-white-4"> */}
                    {/*    {description} */}
                    {/* </h2> */}
                </div>
            </figure>
        </div>
    );
};
export default PodcastCard;
