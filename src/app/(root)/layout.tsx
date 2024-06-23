import { ReactNode } from 'react';

import LeftSidebar from '@/components/LeftSidebar/LeftSidebar';

const RootLayout = ({ children }: { children: ReactNode }) => {
    return (
        <div className="relative flex flex-col">
            <main className="relative flex bg-black-3">
                <LeftSidebar />
                <section className="flex flex-1 min-h-screen flex-col px-4 sm:px-14">
                    <div className="mx-auto flex w-full max-w-5xl flex-col max-sm:px-4">
                        <div className="flex flex-col md:pb-14">{children}</div>
                    </div>
                </section>
            </main>
        </div>
    );
};

export default RootLayout;
