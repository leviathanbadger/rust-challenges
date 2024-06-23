import { Component, OnInit, signal } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterOutlet } from '@angular/router';
import { invoke } from "@tauri-apps/api/tauri";
import { Observable, shareReplay } from 'rxjs';
import { observableFromTauriEvent } from '../utils';

@Component({
    selector: 'app-root',
    standalone: true,
    imports: [CommonModule, RouterOutlet],
    templateUrl: './app.component.html',
    styleUrl: './app.component.css'
})
export class AppComponent implements OnInit {
    greetingMessage = signal('');

    timer$!: Observable<number>;

    ngOnInit(): void {
        this.timer$ = observableFromTauriEvent<number>('updateTimer').pipe(
            shareReplay(1)
        );
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
