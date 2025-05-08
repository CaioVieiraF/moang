import styled from 'styled-components'

export const BannerContainer = styled.div`
  max-width: 76rem;
  height: 100%;
  width: 100%;
  border-radius: 3px;
  background: ${props => props.theme['secondary-700']};
  box-shadow: 4px 4px 10px rgba(0, 0, 0, 0.6);
  padding: 3rem;

  display: flex;
  flex-direction: row;
`
export const BannerImg = styled.img`
  max-height: 15rem;
  border-radius: 6px;
  box-shadow: 2px 2px 8px rgba(0, 0, 0, 0.4);
`

export const BannerInfoArea = styled.div`
  margin: auto;
  padding: 2rem;
  font-size: 20px;
  line-height: 1.5rem;
  color: ${props => props.theme['text']};
  margin-bottom: 0;

  header, &>p {
    margin-bottom: 1rem;
  }

  p {
    color: ${props => props.theme['text-200']};
  }
`

export const BannerSocial = styled.div`
  display: flex;
  flex-direction: row;
  gap: 1rem;

  a {
    display: flex;
    flex-direction: row;
    gap: 0.25rem;
    padding: 0.25rem;
    border-radius: 3px;

    font-size: 18px;
    color: ${props => props.theme['text-900']};
    text-decoration: none;
    background: ${props => props.theme.primary};
    
    p {
      color: ${props => props.theme['text-900']};
    }
  }
`
