import { Sidebar } from "./Sidebar";

interface LayoutProps {
  children: React.ReactNode;
  onSettingsClick: () => void;
}

export function Layout({ children, onSettingsClick }: LayoutProps) {
  return (
    <div className="flex h-screen bg-background">
      <Sidebar onSettingsClick={onSettingsClick} />
      <main className="flex-1 flex">{children}</main>
    </div>
  );
}
