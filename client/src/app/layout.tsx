import type { Metadata } from "next";
import "./globals.css";
import { Sidebar } from "@/components/sidebar";

export const metadata: Metadata = {
  title: "Create Next App",
  description: "Generated by create next app",
};

const RootLayout = ({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) => {
  return (
    <html lang="en">
      <body className="bg-black-dark text-white relative z-0 flex h-full w-full overflow-hidden">
        <nav className="bg-black border-r border-black-light w-64 flex-shrink-0 overflow-x-hidden h-screen">
          <Sidebar />
        </nav>
        <main className="relative flex h-full w-full flex-1 p-4">
          {children}
        </main>
      </body>
    </html>
  );
};

export default RootLayout;
