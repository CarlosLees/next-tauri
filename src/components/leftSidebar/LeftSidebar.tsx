'use client';

import Link from 'next/link';
import Image from 'next/image';

import { usePathname } from 'next/navigation';

import { sidebarLinks } from '@/constants';
import { cn } from '@/lib/utils';

const LeftSidebar = () => {
    const pathname = usePathname();
    return (
        <section className="left_sidebar">
            <nav className="flex flex-col gap-6">
                <Link
                    href="/"
                    className="flex cursor-pointer items-center gap-4 max-lg:justify-center pb-4"
                >
                    <Image src="/icons/logo.svg" alt="logo" width={23} height={27} />
                    <h1 className="text-24 font-extrabold text-white-1 max-lg:hidden">Podcast</h1>
                </Link>
            </nav>
            {sidebarLinks.map((item) => {
                const isActive = pathname === item.route || pathname.startsWith(`${item.route}/`);
                return (
                    <Link
                        href={item.route}
                        key={item.label}
                        className={cn('flex gap-5 items-center py-4 max-lg:px-4 justify-start', {
                            'bg-nav-focus border-r-4 border-orange-1': isActive,
                        })}
                    >
                        <Image src={item.imageUrl} alt={item.label} width={24} height={24} />
                        <p className="text-16 font-bold max-lg:hidden">{item.label}</p>
                    </Link>
                );
            })}
        </section>
    );
};

export default LeftSidebar;
