import { CommonModule } from "@angular/common";
import { Component, OnInit } from "@angular/core";
import { ActivatedRoute } from "@angular/router";
import { Observable, map } from "rxjs";

@Component({
    standalone: true,
    imports: [CommonModule],
    templateUrl: './not-found.component.html',
    styleUrls: ['./not-found.component.scss']
})
export class NotFoundComponent implements OnInit {
    constructor(
        private activatedRoute: ActivatedRoute
    ) { }

    outlet = '';
    url$!: Observable<string>;

    ngOnInit(): void {
        this.outlet = this.activatedRoute.outlet;
        this.url$ = this.activatedRoute.url.pipe(
            map(segments => {
                console.log(`ROUTE NOT FOUND in outlet ${this.activatedRoute.outlet}. url:`, segments);
                return segments.map(segment => segment.toString()).join('/');
            })
        );
    }
}
