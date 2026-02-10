import { useEffect, useState } from "react";
import { tauriService } from "../services/tauri";

export function useFileWatcher(indexPath?: string) {
    const [active, setActive] = useState(false);
    const [events, setEvents] = useState<any[]>([]);

    useEffect(() => {
        let interval: any = null;
        if (indexPath) {
            setActive(true);
            interval = setInterval(async () => {
                try {
                    // Poll backend for new file events
                    const res: any = await tauriService.getFileEvents(indexPath);
                    if (res && Array.isArray(res) && res.length > 0) {
                        setEvents(res);
                    }
                } catch (e) {
                    // ignore
                }
            }, 2000);
        }
        return () => {
            if (interval) clearInterval(interval);
            setActive(false);
        };
    }, [indexPath]);

    return { active, events };
}
