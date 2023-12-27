import { faker } from '@faker-js/faker';

export type User = {
  id: string;
};

export const fakeUser = () => ({
  id: faker.string.uuid(),
});

export type Group = {
  id: string;
  created_at: Date;
  name: string;
  users: User[];
};

export const fakeGroup = () => ({
  id: faker.string.uuid(),
  created_at: faker.date.anytime(),
  name: faker.lorem.word(),
  users: [fakeUser(), fakeUser(), fakeUser()]
})
