'use client';

import { invoke } from '@tauri-apps/api/tauri';

const Myself = () => {
    const getAppList = async () => {
        invoke('app_list').then((res) => {});
    };

    return (
        <div>
            <button onClick={getAppList}>确定</button>
        </div>
    );
};

export default Myself;
