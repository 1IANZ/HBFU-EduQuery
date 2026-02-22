const BASE_URL = "https://hbfu.karpov.cn/api";
// const BASE_URL = "http://127.0.0.1:8000/api";
const API = {
  auth: {
    login_jwxt: `${BASE_URL}/auth/login/jwxt`,
    vpn: {
      manual: `${BASE_URL}/auth/login/vpn/manual`,
      qrcode: `${BASE_URL}/auth/login/vpn/qrcode`,
    },
    captcha: `${BASE_URL}/auth/captcha`,
    qrcode: {
      uuid: `${BASE_URL}/auth/qrcode/uuid`,
      status: (sessionId) =>
        `${BASE_URL}/auth/qrcode/status?sessionId=${sessionId}`,
      login: `${BASE_URL}/auth/qrcode/login`,
    },
  },

  user: {
    info: (studentId) => `${BASE_URL}/user/info/${studentId}`,
    semester: (studentId, isAll = false) =>
      `${BASE_URL}/user/semester/${studentId}?is_all=${isAll}`,
    score: (studentId, semester = null) => {
      const url = `${BASE_URL}/user/score/${studentId}`;
      return semester ? `${url}?semester=${semester}` : url;
    },
    exam: (studentId, semester = null) => {
      const url = `${BASE_URL}/user/exam/${studentId}`;
      return semester ? `${url}?semester=${semester}` : url;
    },
    elective: (studentId, semester = null) => {
      const url = `${BASE_URL}/user/elective/${studentId}`;
      return semester ? `${url}?semester=${semester}` : url;
    },
    plan: (studentId) => `${BASE_URL}/user/plan/${studentId}`,
    course: (studentId, semester = null) => {
      const url = `${BASE_URL}/user/course/${studentId}`;
      return semester ? `${url}?semester=${semester}` : url;
    },
    dekt: (studentId) => `${BASE_URL}/user/dekt/${studentId}`,
    dektDetail: (studentId, operationId = null) => {
      const url = `${BASE_URL}/user/dekt/detail/${studentId}`;
      return operationId ? `${url}?operationId=${operationId}` : url;
    },
  },

  image: {
    get: (filename) => `${BASE_URL}/image/${filename}`,
  },

  system: {
    config: `${BASE_URL}/config`,
  },
};

export default API;
export { BASE_URL };
