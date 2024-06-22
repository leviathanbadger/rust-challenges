import { Component, OnDestroy, OnInit, signal } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterOutlet } from '@angular/router';
import { invoke } from "@tauri-apps/api/tauri";
import { listen, UnlistenFn, Event } from "@tauri-apps/api/event";

@Component({
    selector: 'app-root',
    standalone: true,
    imports: [CommonModule, RouterOutlet],
    templateUrl: './app.component.html',
    styleUrl: './app.component.css'
})
export class AppComponent implements OnInit, OnDestroy {
    greetingMessage = signal('');

    timer = signal(0);
    unlistenFn: UnlistenFn | null = null;

    updateTimer(timer: number) {
        this.timer.set(timer);
    }

    ngOnInit(): void {
        listen('updateTimer', (event: Event<number>) => this.updateTimer(event.payload)).then(unlistenFn => this.unlistenFn = unlistenFn);
    }
    ngOnDestroy(): void {
        this.unlistenFn?.();
        this.unlistenFn = null;
    }

    async greet(event: SubmitEvent, name: string): Promise<void> {
        event.preventDefault();

        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        try {
            let text = await invoke<string>("greet", { name });
            this.greetingMessage.set(text);
        }
        catch (err) {
            this.greetingMessage.set(`${err}`);
        }
    }
}
