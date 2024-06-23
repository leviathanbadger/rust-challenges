import { Routes } from "@angular/router";
import { LayoutComponent } from "../layout/layout.component";
import { primaryRoutes } from "../primary/primary.routes";
import { propertiesRoutes } from "../properties/properties.routes";
import { treeRoutes } from "../tree/tree.routes";
import { NotFoundComponent } from "../not-found/not-found.component";

export const routes: Routes = [{
    path: '',
    component: LayoutComponent,
    children: [{
        path: '',
        outlet: 'primary',
        children: primaryRoutes
    }, {
        path: '',
        outlet: 'properties',
        children: propertiesRoutes
    }, {
        path: '',
        outlet: 'tree',
        children: treeRoutes
    }, {
        path: '**',
        outlet: 'primary',
        component: NotFoundComponent
    }, {
        path: '**',
        outlet: 'properties',
        component: NotFoundComponent
    }, {
        path: '**',
        outlet: 'tree',
        component: NotFoundComponent
    }]
}];
