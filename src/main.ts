import { App } from 'cdktf';

import { HomeStuffStack } from './HomeStuffStack';

const app = new App();
new HomeStuffStack(app, 'home-stuff-local', {
  env: 'local'
});

app.synth();
