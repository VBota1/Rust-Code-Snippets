#[macro_use]
extern crate log;
use log::LevelFilter;
extern crate simple_logging;

pub fn initiate_logging(log_name: Option<String>) -> Result<String,String>
{
    let log_file = log_name.unwrap_or(format!("trace.log"));
    match simple_logging::log_to_file(log_file, LevelFilter::Trace) {
        Ok(_) => {
            Ok(format!("Logging initiated"))
        },
        Err(error) => {
            Err(format!("error {} occured while trying to intiate logging",error))
        },
    }
}

pub fn error(message: String) -> String {
    error!("{}", message.clone());
    message
}

pub fn warn(message: String) -> String {
    warn!("{}", message);
    message
}

pub fn info(message: String) -> String {
    info!("{}", message);
    message
}

pub fn debug(message: String)-> String {
    debug!("{}", message);
    message
}

pub fn trace(message: String) -> String {
    trace!("{}", message);
    message
}

/*LICENSE for extern crate simple_logging
Copyright 2017 Isabela Schulze
Redistribution and use in source and binary forms, with or without modification, are permitted provided that the following conditions are met:
1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following disclaimer.
2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the following disclaimer in the documentation and/or other materials provided with the distribution.
3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote products derived from this software without specific prior written permission.
THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/
