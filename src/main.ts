import { App } from 'cdktf';

import { HomeStuffStack } from './HomeStuffStack';

const app = new App();

new HomeStuffStack(app, 'home-stuff-local', {
  env: 'local'
});

new HomeStuffStack(app, 'home-stuff-prod', {
  env: 'prod-eu'
});

new HomeStuffStack(app, 'home-stuff-staging', {
  env: 'prod-eu'
});

app.synth();
