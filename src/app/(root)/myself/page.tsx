'use client';

import { useState } from 'react';
import Image from 'next/image';

const imageLoader = ({ src, width }: { src: string; width: number }) => {
    const result = src.split('/').find((res) => res.search('.icns') !== -1);
    let url = '';
    if (result) {
        url = result.replace('.icns', '.png');
    }

    console.log('result:', url);
    return `/${url}?w=${width}`;
};

const Myself = () => {
    const [imageUrl, setImageUrl] = useState('');

    const getAppList = async () => {
        setImageUrl('/Applications/RustRover.app/Contents/Resources/rustrover.icns');
    };

    return (
        <div>
            <button onClick={getAppList}>确定</button>
            <Image loader={imageLoader} src={imageUrl} alt="image" width={100} height={100} />
        </div>
    );
};

export default Myself;
