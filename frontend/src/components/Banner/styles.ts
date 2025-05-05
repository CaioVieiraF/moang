import styled from 'styled-components'

export const BannerContainer = styled.div`
  max-width: 76rem;
  height: 100%;
  width: 100%;
  border-radius: 3px;
  background: ${props => props.theme['primary']};
  padding: 5rem;

  display: flex;
  flex-direction: row;
`
export const BannerImg = styled.img`
  max-height: 15rem;
`

export const BannerInfoArea = styled.div`
  margin: auto;
  padding: 2rem;
  font-size: 20px;
  line-height: 1.5rem;
  color: ${props => props.theme['text-700']};
  margin-bottom: 0;

  header, p {
    margin-bottom: 1rem;
  }
`

export const BannerSocial = styled.div`
  display: flex;
  flex-direction: row;

  a {
    font-size: 22px;
    color: black;
    text-decoration: none;
  }
`
