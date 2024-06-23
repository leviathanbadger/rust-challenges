import { Routes } from "@angular/router";
import { ParametersComponent } from "./parameters/parameters.component";

export const propertiesRoutes: Routes =  [{
    path: '',
    pathMatch: 'full',
    redirectTo: 'parameters'
}, {
    path: 'parameters',
    component: ParametersComponent
}];
