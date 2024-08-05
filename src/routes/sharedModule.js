let sharedData = {
    token: '',
    refershToken: 'light',
    name: ''
};

const BASE_API_URL='https://p1.aha.id/'
const NEW_BASE_API_URL='https://apigateway-cms.aha.id/cms'
export const AHAMART_API_URL='https://ahamart-apigateway.service-aha.id/cms'

export function setToken(token) {
    sharedData.token = token;
}

export function getToken() {
    return sharedData.token;
}

export function setRefreshToken(refershToken) {
    sharedData.refershToken = refershToken;
}

export function getRefreshToken() {
    return sharedData.refershToken;
}

export function setName(name) {
    sharedData.name = name;
}

export function getName() {
    return sharedData.name;
}