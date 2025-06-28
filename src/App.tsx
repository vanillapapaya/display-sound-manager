import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { appWindow } from '@tauri-apps/api/window';
import './App.css';

interface DisplayInfo {
  id: number;
  name: string;
  width: number;
  height: number;
  x: number;
  y: number;
  scale_factor: number;
  is_primary: boolean;
  rotation: number;
}

interface AudioDevice {
  id: string;
  name: string;
  is_default: boolean;
  device_type: string;
}

interface AudioSettings {
  output_device?: string;
  input_device?: string;
  output_volume: number;
  input_volume: number;
}

interface Profile {
  id: string;
  name: string;
  displays: DisplayInfo[];
  audio_settings: AudioSettings;
  created_at: string;
}

function App() {
  const [displays, setDisplays] = useState<DisplayInfo[]>([]);
  const [audioDevices, setAudioDevices] = useState<AudioDevice[]>([]);
  const [profiles, setProfiles] = useState<Profile[]>([]);
  const [selectedProfile, setSelectedProfile] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    loadSystemInfo();
    loadProfiles();
    
    // 창 표시
    appWindow.show();
  }, []);

  const loadSystemInfo = async () => {
    try {
      const [displayData, audioData] = await Promise.all([
        invoke<DisplayInfo[]>('get_displays'),
        invoke<AudioDevice[]>('get_audio_devices')
      ]);
      
      setDisplays(displayData);
      setAudioDevices(audioData);
    } catch (error) {
      console.error('시스템 정보 로드 실패:', error);
    }
  };

  const loadProfiles = async () => {
    try {
      const profileData = await invoke<Profile[]>('get_profiles');
      setProfiles(profileData);
    } catch (error) {
      console.error('프로필 로드 실패:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const createProfile = async () => {
    const name = prompt('프로필 이름을 입력하세요:');
    if (!name) return;

    const outputDevice = audioDevices.find(d => d.device_type === 'output' && d.is_default);
    const inputDevice = audioDevices.find(d => d.device_type === 'input' && d.is_default);

    const newProfile: Profile = {
      id: Date.now().toString(),
      name,
      displays: [...displays],
      audio_settings: {
        output_device: outputDevice?.id,
        input_device: inputDevice?.id,
        output_volume: 50,
        input_volume: 50,
      },
      created_at: new Date().toISOString(),
    };

    try {
      await invoke('save_profile', { profile: newProfile });
      await loadProfiles();
    } catch (error) {
      console.error('프로필 저장 실패:', error);
    }
  };

  const applyProfile = async (profileId: string) => {
    try {
      await invoke('apply_profile', { profileId });
      setSelectedProfile(profileId);
      alert('프로필이 적용되었습니다!');
    } catch (error) {
      console.error('프로필 적용 실패:', error);
      alert('프로필 적용에 실패했습니다.');
    }
  };

  const deleteProfile = async (profileId: string) => {
    if (!confirm('이 프로필을 삭제하시겠습니까?')) return;

    try {
      await invoke('delete_profile', { profileId });
      await loadProfiles();
    } catch (error) {
      console.error('프로필 삭제 실패:', error);
    }
  };

  if (isLoading) {
    return <div className="loading">로딩 중...</div>;
  }

  return (
    <div className="container">
      <h1>🖥️ Display & Sound Manager</h1>

      <div className="section">
        <h2>현재 시스템 구성</h2>
        
        <div className="system-info">
          <div className="displays-section">
            <h3>디스플레이</h3>
            {displays.map((display, index) => (
              <div key={display.id} className="info-card">
                <h4>모니터 {index + 1} {display.is_primary && '(주 모니터)'}</h4>
                <p>이름: {display.name}</p>
                <p>해상도: {display.width} × {display.height}</p>
                <p>위치: ({display.x}, {display.y})</p>
                <p>배율: {display.scale_factor}x</p>
                <p>회전: {display.rotation}°</p>
              </div>
            ))}
          </div>

          <div className="audio-section">
            <h3>오디오 장치</h3>
            <div className="audio-devices">
              <h4>출력 장치</h4>
              {audioDevices
                .filter(d => d.device_type === 'output')
                .map(device => (
                  <div key={device.id} className="audio-device">
                    {device.name} {device.is_default && '(기본)'}
                  </div>
                ))}
              
              <h4>입력 장치</h4>
              {audioDevices
                .filter(d => d.device_type === 'input')
                .map(device => (
                  <div key={device.id} className="audio-device">
                    {device.name} {device.is_default && '(기본)'}
                  </div>
                ))}
            </div>
          </div>
        </div>

        <button className="btn-primary" onClick={createProfile}>
          현재 설정을 프로필로 저장
        </button>
      </div>

      <div className="section">
        <h2>저장된 프로필</h2>
        
        {profiles.length === 0 ? (
          <p>저장된 프로필이 없습니다.</p>
        ) : (
          <div className="profiles-grid">
            {profiles.map(profile => (
              <div 
                key={profile.id} 
                className={`profile-card ${selectedProfile === profile.id ? 'active' : ''}`}
              >
                <h3>{profile.name}</h3>
                <p>디스플레이: {profile.displays.length}개</p>
                <p>생성일: {new Date(profile.created_at).toLocaleDateString()}</p>
                
                <div className="profile-details">
                  <small>
                    출력: {profile.audio_settings.output_device || '기본'}<br/>
                    입력: {profile.audio_settings.input_device || '기본'}
                  </small>
                </div>
                
                <div className="profile-actions">
                  <button 
                    className="btn-apply" 
                    onClick={() => applyProfile(profile.id)}
                  >
                    적용
                  </button>
                  <button 
                    className="btn-delete" 
                    onClick={() => deleteProfile(profile.id)}
                  >
                    삭제
                  </button>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}

export default App;