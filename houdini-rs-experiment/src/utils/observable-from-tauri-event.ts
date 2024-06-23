import { listen, Event as TauriEvent, UnlistenFn } from "@tauri-apps/api/event";
import { Observable } from "rxjs";

export function observableFromTauriEvent<T>(eventName: string): Observable<T> {
    return new Observable(subscriber => {
        let isClosed = false;
        let unlistenFn: UnlistenFn | null = null;

        listen('updateTimer', (event: TauriEvent<T>) => subscriber.next(event.payload))
            .then(unlisten => {
                unlistenFn = unlisten;
                if (isClosed) {
                    unlisten();
                }
            });

        return () => {
            isClosed = true;
            unlistenFn?.();
        };
    });
}
